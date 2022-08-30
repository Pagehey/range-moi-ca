export enum CommandResponseStatus {
  Success,
  Failure,
}

export interface CommandResponse {
  status: string,
  message: string
}

export function CommandSuccess(status: string) {
  return status === CommandResponseStatus[CommandResponseStatus.Success];
}

export function CommandFailure(status: string) {
  return status === CommandResponseStatus[CommandResponseStatus.Failure]
}
