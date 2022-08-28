import type { ServerInfo } from "./api_types";

type State = {
  isConnected: boolean;
  servers: {[id: string] : ServerInfo},
  testVar: number;
}

export default State;
