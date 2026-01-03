import * as grpc from "@grpc/grpc-js";
import * as protoLoader from "@grpc/proto-loader";
import { AuthProto, AuthServiceClient } from "./auth-grpc.types";

const protoPath = process.cwd() + "/proto/auth.proto";

const packageDef = protoLoader.loadSync(protoPath, {
  keepCase: true,
  longs: String,
  enums: String,
  defaults: true,
});

const proto = grpc.loadPackageDefinition(
  packageDef
) as unknown as AuthProto;

export const authClient = new proto.auth.AuthService(
  process.env.AUTH_GRPC_URL || "localhost:50051",
  grpc.credentials.createInsecure()
) as unknown as AuthServiceClient;