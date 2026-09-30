package com.sicpa.javata;

import com.fasterxml.jackson.databind.ObjectMapper;

public abstract class IPCResponse {
	public static Result<IPCSuccessResponse, IPCFailureResponse> Success(Object data) {
		ObjectMapper mapper = new ObjectMapper();
		return Result.wrapping(() -> mapper.writeValueAsString(data)).map(d -> new IPCSuccessResponse(d)).mapErr(e -> new IPCFailureResponse(e));
	}
	public static Result<IPCFailureResponse, IPCFailureResponse> Fail(Object data) {
		ObjectMapper mapper = new ObjectMapper();
		return Result.wrapping(() -> mapper.writeValueAsString(data)).map(d -> new IPCFailureResponse(d)).mapErr(e -> new IPCFailureResponse(e));
	}

	public static final class IPCSuccessResponse extends IPCResponse {
		private Object data;

		private IPCSuccessResponse(String data) {
			this.data = data;
		}

		public Object getData() {
			return data;
		}
	}

	public static final class IPCFailureResponse extends IPCResponse {
		private Object error;

		public IPCFailureResponse(Object error) {
			this.error = error;
		}

		public Object getError() {
			return error;
		}
	}
}
