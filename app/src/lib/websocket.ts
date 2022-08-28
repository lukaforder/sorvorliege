import { encode_cmd } from "$lib/util/cmd";
import type { ClientCommands, ServerCommands } from "./modals/api_types";

export interface ISharedSocket {
  open(id: number): Promise<void>;
  close(id: number): void;

  send(id: number, command: ClientCommands): Promise<void>;

  onopen(id: number, cb: () => void): void;
  onclose(id: number, cb: () => void): void;
  onerror(id: number, cb: (err: Error) => void): void;
  onmessage(id: number, cb: (event: ServerCommands) => void): void;
}

let socket: SharedSocket | null;
let counter = 0;

export class SharedSocket implements ISharedSocket {
  private socket: WebSocket;
  private users: Set<number> = new Set();
  private on: { [id: number]: { 
      open?: () => void,
      close?: () => void,
      error?: (err: Error) => void,
      message?: (event: ServerCommands) => void,
    }
  } = {};
  constructor (url: string, options: string[]) {
    this.socket = new WebSocket(url, options);
    this.socket.onmessage = async (event) => {
      try {
        const command = JSON.parse(atob(await event.data.text()));
        for (const id of this.users) {
          const cb = this.on[id]["message"];
          if (cb) cb(command);
        } 
      } catch (e) {
        console.error(e);
        console.error(`Unknown message: '${event.data}'`);
      }
    };
    
  }
  async open(id: number): Promise<void> {
    this.users.add(id);
  }
  close(id: number): void {
    this.users.delete(id);
    delete this.on[id];
    if(this.users.size === 0) {
      this.socket.close(undefined, "no more consumers");
    }
  }
  async send(id: number, command: ClientCommands): Promise<void> {
    if(this.socket.readyState !== WebSocket.OPEN) await this.open(id);
    this.socket.send(encode_cmd(command));
  }
  onopen(id: number, cb: () => void): void {
    if(!this.on[id]) this.on[id] = {};
    this.on[id]["open"] = cb;
  }
  onclose(id: number, cb: () => void): void {
    if(!this.on[id]) this.on[id] = {};
    this.on[id]["close"] = cb;
  }
  onerror(id: number, cb: (err: Error) => void): void {
    if(!this.on[id]) this.on[id] = {};
    this.on[id]["error"] = cb;
  }
  onmessage(id: number, cb: (event: ServerCommands) => void): void {
    if(!this.on[id]) this.on[id] = {};
    this.on[id]["message"] = cb;
  }
}

export const get_socket = (url: string, socketOptions: string[]): [number, SharedSocket] => {
  if(!socket)
    socket = new SharedSocket(url, socketOptions);
  return [counter++, socket];
}