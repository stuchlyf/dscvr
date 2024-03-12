/* eslint-disable */
import * as _m0 from "protobufjs/minimal";
import { Empty } from "./proto_utils";
import Long = require("long");

export const protobufPackage = "file_indexer";

export interface FindDuplicatedFilesResponse {
  files: DuplicatedFile[];
}

export interface DuplicatedFile {
  paths: string[];
  aggregatedSize: number;
  duplicates: number;
  hash: string;
}

export interface FindDuplicatedFilesQuery {
  startingAtPath?: string | undefined;
}

export interface SearchFileByContentsQuery {
  query: string;
}

export interface SearchFileResponse {
  path: string[];
}

export interface IndexFileQuery {
  scannedFiles: ScannedFile[];
}

export interface ScannedFile {
  path: string;
  readable: boolean;
  /**
   * uint64 scanned_at = 4;
   *  uint64 size = 5;
   */
  hash: string;
}

function createBaseFindDuplicatedFilesResponse(): FindDuplicatedFilesResponse {
  return { files: [] };
}

export const FindDuplicatedFilesResponse = {
  encode(message: FindDuplicatedFilesResponse, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    for (const v of message.files) {
      DuplicatedFile.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): FindDuplicatedFilesResponse {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseFindDuplicatedFilesResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.files.push(DuplicatedFile.decode(reader, reader.uint32()));
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): FindDuplicatedFilesResponse {
    return {
      files: globalThis.Array.isArray(object?.files) ? object.files.map((e: any) => DuplicatedFile.fromJSON(e)) : [],
    };
  },

  toJSON(message: FindDuplicatedFilesResponse): unknown {
    const obj: any = {};
    if (message.files?.length) {
      obj.files = message.files.map((e) => DuplicatedFile.toJSON(e));
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<FindDuplicatedFilesResponse>, I>>(base?: I): FindDuplicatedFilesResponse {
    return FindDuplicatedFilesResponse.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<FindDuplicatedFilesResponse>, I>>(object: I): FindDuplicatedFilesResponse {
    const message = createBaseFindDuplicatedFilesResponse();
    message.files = object.files?.map((e) => DuplicatedFile.fromPartial(e)) || [];
    return message;
  },
};

function createBaseDuplicatedFile(): DuplicatedFile {
  return { paths: [], aggregatedSize: 0, duplicates: 0, hash: "" };
}

export const DuplicatedFile = {
  encode(message: DuplicatedFile, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    for (const v of message.paths) {
      writer.uint32(10).string(v!);
    }
    if (message.aggregatedSize !== 0) {
      writer.uint32(16).uint64(message.aggregatedSize);
    }
    if (message.duplicates !== 0) {
      writer.uint32(24).uint64(message.duplicates);
    }
    if (message.hash !== "") {
      writer.uint32(34).string(message.hash);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): DuplicatedFile {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseDuplicatedFile();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.paths.push(reader.string());
          continue;
        case 2:
          if (tag !== 16) {
            break;
          }

          message.aggregatedSize = longToNumber(reader.uint64() as Long);
          continue;
        case 3:
          if (tag !== 24) {
            break;
          }

          message.duplicates = longToNumber(reader.uint64() as Long);
          continue;
        case 4:
          if (tag !== 34) {
            break;
          }

          message.hash = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): DuplicatedFile {
    return {
      paths: globalThis.Array.isArray(object?.paths) ? object.paths.map((e: any) => globalThis.String(e)) : [],
      aggregatedSize: isSet(object.aggregatedSize) ? globalThis.Number(object.aggregatedSize) : 0,
      duplicates: isSet(object.duplicates) ? globalThis.Number(object.duplicates) : 0,
      hash: isSet(object.hash) ? globalThis.String(object.hash) : "",
    };
  },

  toJSON(message: DuplicatedFile): unknown {
    const obj: any = {};
    if (message.paths?.length) {
      obj.paths = message.paths;
    }
    if (message.aggregatedSize !== 0) {
      obj.aggregatedSize = Math.round(message.aggregatedSize);
    }
    if (message.duplicates !== 0) {
      obj.duplicates = Math.round(message.duplicates);
    }
    if (message.hash !== "") {
      obj.hash = message.hash;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<DuplicatedFile>, I>>(base?: I): DuplicatedFile {
    return DuplicatedFile.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<DuplicatedFile>, I>>(object: I): DuplicatedFile {
    const message = createBaseDuplicatedFile();
    message.paths = object.paths?.map((e) => e) || [];
    message.aggregatedSize = object.aggregatedSize ?? 0;
    message.duplicates = object.duplicates ?? 0;
    message.hash = object.hash ?? "";
    return message;
  },
};

function createBaseFindDuplicatedFilesQuery(): FindDuplicatedFilesQuery {
  return { startingAtPath: undefined };
}

export const FindDuplicatedFilesQuery = {
  encode(message: FindDuplicatedFilesQuery, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.startingAtPath !== undefined) {
      writer.uint32(10).string(message.startingAtPath);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): FindDuplicatedFilesQuery {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseFindDuplicatedFilesQuery();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.startingAtPath = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): FindDuplicatedFilesQuery {
    return { startingAtPath: isSet(object.startingAtPath) ? globalThis.String(object.startingAtPath) : undefined };
  },

  toJSON(message: FindDuplicatedFilesQuery): unknown {
    const obj: any = {};
    if (message.startingAtPath !== undefined) {
      obj.startingAtPath = message.startingAtPath;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<FindDuplicatedFilesQuery>, I>>(base?: I): FindDuplicatedFilesQuery {
    return FindDuplicatedFilesQuery.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<FindDuplicatedFilesQuery>, I>>(object: I): FindDuplicatedFilesQuery {
    const message = createBaseFindDuplicatedFilesQuery();
    message.startingAtPath = object.startingAtPath ?? undefined;
    return message;
  },
};

function createBaseSearchFileByContentsQuery(): SearchFileByContentsQuery {
  return { query: "" };
}

export const SearchFileByContentsQuery = {
  encode(message: SearchFileByContentsQuery, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.query !== "") {
      writer.uint32(10).string(message.query);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): SearchFileByContentsQuery {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseSearchFileByContentsQuery();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.query = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): SearchFileByContentsQuery {
    return { query: isSet(object.query) ? globalThis.String(object.query) : "" };
  },

  toJSON(message: SearchFileByContentsQuery): unknown {
    const obj: any = {};
    if (message.query !== "") {
      obj.query = message.query;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<SearchFileByContentsQuery>, I>>(base?: I): SearchFileByContentsQuery {
    return SearchFileByContentsQuery.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<SearchFileByContentsQuery>, I>>(object: I): SearchFileByContentsQuery {
    const message = createBaseSearchFileByContentsQuery();
    message.query = object.query ?? "";
    return message;
  },
};

function createBaseSearchFileResponse(): SearchFileResponse {
  return { path: [] };
}

export const SearchFileResponse = {
  encode(message: SearchFileResponse, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    for (const v of message.path) {
      writer.uint32(10).string(v!);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): SearchFileResponse {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseSearchFileResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.path.push(reader.string());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): SearchFileResponse {
    return { path: globalThis.Array.isArray(object?.path) ? object.path.map((e: any) => globalThis.String(e)) : [] };
  },

  toJSON(message: SearchFileResponse): unknown {
    const obj: any = {};
    if (message.path?.length) {
      obj.path = message.path;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<SearchFileResponse>, I>>(base?: I): SearchFileResponse {
    return SearchFileResponse.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<SearchFileResponse>, I>>(object: I): SearchFileResponse {
    const message = createBaseSearchFileResponse();
    message.path = object.path?.map((e) => e) || [];
    return message;
  },
};

function createBaseIndexFileQuery(): IndexFileQuery {
  return { scannedFiles: [] };
}

export const IndexFileQuery = {
  encode(message: IndexFileQuery, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    for (const v of message.scannedFiles) {
      ScannedFile.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): IndexFileQuery {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseIndexFileQuery();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.scannedFiles.push(ScannedFile.decode(reader, reader.uint32()));
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): IndexFileQuery {
    return {
      scannedFiles: globalThis.Array.isArray(object?.scannedFiles)
        ? object.scannedFiles.map((e: any) => ScannedFile.fromJSON(e))
        : [],
    };
  },

  toJSON(message: IndexFileQuery): unknown {
    const obj: any = {};
    if (message.scannedFiles?.length) {
      obj.scannedFiles = message.scannedFiles.map((e) => ScannedFile.toJSON(e));
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<IndexFileQuery>, I>>(base?: I): IndexFileQuery {
    return IndexFileQuery.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<IndexFileQuery>, I>>(object: I): IndexFileQuery {
    const message = createBaseIndexFileQuery();
    message.scannedFiles = object.scannedFiles?.map((e) => ScannedFile.fromPartial(e)) || [];
    return message;
  },
};

function createBaseScannedFile(): ScannedFile {
  return { path: "", readable: false, hash: "" };
}

export const ScannedFile = {
  encode(message: ScannedFile, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.path !== "") {
      writer.uint32(10).string(message.path);
    }
    if (message.readable === true) {
      writer.uint32(16).bool(message.readable);
    }
    if (message.hash !== "") {
      writer.uint32(26).string(message.hash);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ScannedFile {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseScannedFile();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.path = reader.string();
          continue;
        case 2:
          if (tag !== 16) {
            break;
          }

          message.readable = reader.bool();
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.hash = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ScannedFile {
    return {
      path: isSet(object.path) ? globalThis.String(object.path) : "",
      readable: isSet(object.readable) ? globalThis.Boolean(object.readable) : false,
      hash: isSet(object.hash) ? globalThis.String(object.hash) : "",
    };
  },

  toJSON(message: ScannedFile): unknown {
    const obj: any = {};
    if (message.path !== "") {
      obj.path = message.path;
    }
    if (message.readable === true) {
      obj.readable = message.readable;
    }
    if (message.hash !== "") {
      obj.hash = message.hash;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ScannedFile>, I>>(base?: I): ScannedFile {
    return ScannedFile.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ScannedFile>, I>>(object: I): ScannedFile {
    const message = createBaseScannedFile();
    message.path = object.path ?? "";
    message.readable = object.readable ?? false;
    message.hash = object.hash ?? "";
    return message;
  },
};

export interface FileIndexer {
  IndexFile(request: IndexFileQuery): Promise<Empty>;
  SearchFileByContents(request: SearchFileByContentsQuery): Promise<SearchFileResponse>;
  FindDuplicatedFiles(request: FindDuplicatedFilesQuery): Promise<FindDuplicatedFilesResponse>;
}

export const FileIndexerServiceName = "file_indexer.FileIndexer";
export class FileIndexerClientImpl implements FileIndexer {
  private readonly rpc: Rpc;
  private readonly service: string;
  constructor(rpc: Rpc, opts?: { service?: string }) {
    this.service = opts?.service || FileIndexerServiceName;
    this.rpc = rpc;
    this.IndexFile = this.IndexFile.bind(this);
    this.SearchFileByContents = this.SearchFileByContents.bind(this);
    this.FindDuplicatedFiles = this.FindDuplicatedFiles.bind(this);
  }
  IndexFile(request: IndexFileQuery): Promise<Empty> {
    const data = IndexFileQuery.encode(request).finish();
    const promise = this.rpc.request(this.service, "IndexFile", data);
    return promise.then((data) => Empty.decode(_m0.Reader.create(data)));
  }

  SearchFileByContents(request: SearchFileByContentsQuery): Promise<SearchFileResponse> {
    const data = SearchFileByContentsQuery.encode(request).finish();
    const promise = this.rpc.request(this.service, "SearchFileByContents", data);
    return promise.then((data) => SearchFileResponse.decode(_m0.Reader.create(data)));
  }

  FindDuplicatedFiles(request: FindDuplicatedFilesQuery): Promise<FindDuplicatedFilesResponse> {
    const data = FindDuplicatedFilesQuery.encode(request).finish();
    const promise = this.rpc.request(this.service, "FindDuplicatedFiles", data);
    return promise.then((data) => FindDuplicatedFilesResponse.decode(_m0.Reader.create(data)));
  }
}

interface Rpc {
  request(service: string, method: string, data: Uint8Array): Promise<Uint8Array>;
}

type Builtin = Date | Function | Uint8Array | string | number | boolean | undefined;

export type DeepPartial<T> = T extends Builtin ? T
  : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial<U>>
  : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>>
  : T extends {} ? { [K in keyof T]?: DeepPartial<T[K]> }
  : Partial<T>;

type KeysOfUnion<T> = T extends T ? keyof T : never;
export type Exact<P, I extends P> = P extends Builtin ? P
  : P & { [K in keyof P]: Exact<P[K], I[K]> } & { [K in Exclude<keyof I, KeysOfUnion<P>>]: never };

function longToNumber(long: Long): number {
  if (long.gt(globalThis.Number.MAX_SAFE_INTEGER)) {
    throw new globalThis.Error("Value is larger than Number.MAX_SAFE_INTEGER");
  }
  return long.toNumber();
}

if (_m0.util.Long !== Long) {
  _m0.util.Long = Long as any;
  _m0.configure();
}

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}
