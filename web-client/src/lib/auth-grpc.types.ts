import * as grpc from "@grpc/grpc-js";

export interface LoginRequest {
  email: string;
  password: string;
}

export interface LoginResponse {
  token: string;
}

export interface RegisterRequest {
  email: string;
  password: string;
}

export interface AuthResponse {
  token: string;
}

export interface ValidateTokenRequest {
  token: string;
}

export interface ValidateTokenResponse {
  valid: boolean;
  user_id: string;
  role: string;
}

export interface AuthServiceClient extends grpc.Client {
   register(
    request: RegisterRequest,
    callback: grpc.requestCallback<AuthResponse>
  ): void;

   login(
    request: LoginRequest,
    callback: grpc.requestCallback<LoginResponse>
  ): void;

  validateToken(
    request: ValidateTokenRequest,
    callback: grpc.requestCallback<ValidateTokenResponse>
  ): void;
}

export interface AuthProto {
  auth: {
    AuthService: grpc.ServiceClientConstructor;
  };
}