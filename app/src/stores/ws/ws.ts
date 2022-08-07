import type State from "../../modals/State";

import { browser } from "$app/env";
import { encode_cmd } from "../../util/cmd";
import { handle } from "./commands";

const reopenTimeouts = [2000, 5000, 10000, 30000, 60000];

export interface WSStore<T> extends SvelteStore<T> {
  set(value: State): void,
  increment(amount: number): void,
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
    console.log('not browser');
    return {
      subscribe(subscription: (value: State) => void) {
        subscription(initialValue);
        // eslint-disable-next-line @typescript-eslint/no-empty-function
        return () => {};
      },
      set(value: State) {console.error("not browser");return;},
      increment(amount: number) {console.error("not browser");return;},
    };
  }

  let socket: WebSocket | null;
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

    if (socket) {
      socket.close();
      socket = null;
    }
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

    socket = new WebSocket(url, socketOptions);

    socket.onmessage = async event => {
      try {
        console.log(atob(await event.data.text()));
        const command = JSON.parse(atob(await event.data.text()));
        handle(command, initialValue);
        subscriptions.forEach(subscription => subscription(initialValue));
      } catch (e) {
        console.error(e);
        console.error(`Unknown message: '${event.data}'`);
      }
    };

    socket.onclose = () => reopen();

    openPromise = new Promise((resolve, reject) => {
      if(!socket) return reject(new Error('socket is null'));
      socket.onerror = error => {
        reject(error);
        openPromise = null;
      };
      socket.onopen = () => {
        reopenCount = 0;
        resolve();
        openPromise = null;
      };
    });
    return openPromise;
  }

  return {
    set(value: State) {
      if(!socket) return;
      const send = () => socket?.send(JSON.stringify(value));
      if (socket.readyState !== WebSocket.OPEN) open().then(send);
      else send();
    },
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
    increment(amount: number) {
      if(!socket) return console.error('socket is null');
      const send = () => socket?.send(encode_cmd({type: 'Increment', body: amount}));
      if (socket.readyState !== WebSocket.OPEN) open().then(send);
      else send();
    },
  };
}

export default wsStore;