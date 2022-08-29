import type { ServerCommands } from "$lib/modals/api_types";
import type State from "$lib/modals/State";

export const handle = (cmd: ServerCommands, state: State) => {
  switch (cmd.type) {
    case "ServerInfo":
      cmd.body.forEach(server => {
        state.servers[server.id] = server;
      });
    break;
    default:
      // throw new Error(`Unknown command: ${cmd.type}`);
    break;
  }
}