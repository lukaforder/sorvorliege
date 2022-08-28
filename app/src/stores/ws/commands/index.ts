import type { ServerCommands } from "src/modals/api_types";
import type State from "src/modals/State";

export const handle = (cmd: ServerCommands, state: State) => {
  switch (cmd.type) {
    case "Counter":
      state.testVar = cmd.body;
    break;
    case "ServerInfo":
      state.servers[cmd.body.id] = cmd.body;
    break;
    default:
      throw new Error(`Unknown command: ${cmd.type}`);
    break;
  }
}