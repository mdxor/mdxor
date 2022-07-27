export type SFile = {
  id: string;
  name: string;
  raw: string;
  createdAt: string;
  updatedAt: string;
};

export class StorageBase {
  protected type: string;
  constructor(type: string) {
    this.type = type;
  }

  get(): Promise<string> {
    return Promise.reject(new Error('Not implemented'));
  }

  getList(): Promise<string> {
    return Promise.reject(new Error('Not implemented'));
  }

  update(): Promise<void> {
    return Promise.reject(new Error('Not implemented'));
  }

  remove(): Promise<void> {
    return Promise.reject(new Error('Not implemented'));
  }

  add(): Promise<void> {
    return Promise.reject(new Error('Not implemented'));
  }
}
