import type State from "../../modals/State";

import { browser } from "$app/environment";
import { encode_cmd } from "../../util/cmd";
import { handle } from "./commands";
import type { ClientCommands } from "$lib/modals/api_types";
import { get_socket, type SharedSocket } from "$lib/websocket";

const reopenTimeouts = [2000, 5000, 10000, 30000, 60000];

export interface WSStore<T> extends SvelteStore<T> {
  send(command: ClientCommands): void,
}

/**
 * Create a writable store based on a web-socket.
 * Data is transferred as JSON.
 * Keeps socket open (reopens if closed) as long as there are subscriptions.
 * @param {string} url the WebSocket url
 * @param {any} initialValue store value used before 1st. response from server is present
 * @param {string[]} socketOptions transparently passed to the WebSocket constructor
 * @return {Store}
 */
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function wsStore(url: string, initialValue: State, socketOptions: string[] = []): WSStore<State> {
  if (!browser) {
    return {
      subscribe(subscription: (value: State) => void) {
        subscription(initialValue);
        // eslint-disable-next-line @typescript-eslint/no-empty-function
        return () => {};
      },
      send(_command: ClientCommands) {console.error("not browser");return;},
    };
  }

  const [id, socket] = get_socket(url, socketOptions);

  let openPromise: Promise<void> | null;
  let reopenTimeoutHandler: NodeJS.Timeout | null;

  let reopenCount = 0;
  const subscriptions: Set<(value: State) => void> = new Set();

  function reopenTimeout() {
    const n = reopenCount;
    reopenCount++;
    return reopenTimeouts[
      n >= reopenTimeouts.length - 1 ? reopenTimeouts.length - 1 : n
    ];
  }

  function close() {
    if (reopenTimeoutHandler) {
      clearTimeout(reopenTimeoutHandler);
    }

    socket.close(id);
  }

  function reopen() {
    close();
    if (subscriptions.size > 0) {
      reopenTimeoutHandler = setTimeout(() => open(), reopenTimeout());
    }
  }

  async function open() {
    if (reopenTimeoutHandler) {
      clearTimeout(reopenTimeoutHandler);
      reopenTimeoutHandler = null;
    }

    // we are still in the opening phase
    if(openPromise) {
      return openPromise;
    }

    socket.open(id);

    socket.onmessage(id, async event => {
      handle(event, initialValue);
      subscriptions.forEach(subscription => subscription(initialValue));
    });

    socket.onclose(id, () => reopen());

    openPromise = new Promise((resolve, reject) => {
      if(!socket) return reject(new Error('socket is null'));
      socket.onerror(id, error => {
        reject(error);
        openPromise = null;
      });
      socket.onopen(id, () => {
        reopenCount = 0;
        resolve();
        openPromise = null;
      });
    });
    return openPromise;
  }

  return {
    subscribe(subscription: (value: State) => void) {
      open();
      subscription(initialValue);
      subscriptions.add(subscription);
      return () => {
        subscriptions.delete(subscription);
        if (subscriptions.size === 0) {
          close();
        }
      };
    },
    send(command: ClientCommands) {
      socket.send(id, command);
    }
  };
}

export default wsStore;