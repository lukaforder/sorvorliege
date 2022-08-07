import type {ClientCommands} from '../modals/api_types';

export const encode_cmd = (cmd: ClientCommands) => {
  return btoa(JSON.stringify(cmd));
}