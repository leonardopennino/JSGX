package com.sicpa.javata;

import java.lang.reflect.Method;
import java.util.HashMap;

import com.sicpa.javata.IPCResponse.IPCFailureResponse;
import com.sicpa.javata.IPCResponse.IPCSuccessResponse;

public class SecureMethodHandler extends MethodHandler {

	public SecureMethodHandler(HashMap<String, Method> methods) {
		super(methods);
	}

	@Override
	public Result<IPCSuccessResponse, IPCFailureResponse> handleMessage(IIPCMessage message) {
		if (!this.mappedMethods.containsKey(message.getMethod())) {
			return Result.Err(new IPCFailureResponse("Function not found"));
		}
		var method = this.mappedMethods.get(message.getMethod());
		var methodParameters = method.getParameterTypes();
		if (methodParameters.length == 0) {
			var result = Result
					.wrapping(() -> method.invoke(IPCListener.getTA()))
					.andThen(r -> IPCResponse.Success(r))
					.mapErr((x) -> new IPCFailureResponse("Invocation exception"));
			return result;
		}
		if (message.getParams() == null || message.getParams().size() != methodParameters.length) {
			return Result.Err(new IPCFailureResponse("Incorrect number of parameters"));
		}
		var mappedParams = mapParams(methodParameters, message.getParams());
		return Result.wrapping(() -> method.invoke(IPCListener.getTA(), mappedParams.toArray()))
				.andThen(r -> IPCResponse.Success(r))
				.mapErr(e -> new IPCFailureResponse(e));

	}
}
