import { browser } from "$app/environment";

import type { ClientCommands, Message } from "$lib/modals/api_types";
import { get_socket } from "$lib/websocket";
import type { WSStore } from "./ws";

export interface Logs {
  pages: (Message[])[];
}

export function logStore(url: string, initialValue: Logs, socketOptions: string[] = []): WSStore<Logs> {
  if (!browser) {
    return {
      // eslint-disable-next-line @typescript-eslint/no-empty-function
      subscribe(subscription: (value: Logs) => void) {
        subscription(initialValue);
        // eslint-disable-next-line @typescript-eslint/no-empty-function
        return () => {};
      },
      send() {console.error("not browser");return;},
    }
  }
  const [id, socket] = get_socket(url, socketOptions);
  const subscriptions: Set<(value: Logs) => void> = new Set();

  async function open() {
    socket.open(id);

    socket.onmessage(id, async cmd => {
      switch (cmd.type) {
        case "ServerLogs":
          initialValue.pages[cmd.body.page] = cmd.body.messages;
        break;
        default:
          break;
      }
      subscriptions.forEach(subscription => subscription(initialValue));
    });
  }

  function close() {
    socket.close(id);
  }

  return {
    subscribe(subscription: (value: Logs) => void) {
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
  }
}

export default logStore;