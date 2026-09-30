package com.sicpa.javata;

import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;
import java.util.HashMap;

import com.sicpa.javata.IPCResponse.IPCFailureResponse;
import com.sicpa.javata.IPCResponse.IPCSuccessResponse;

public class AuthSecureMethodHandler extends MethodHandler {

	public AuthSecureMethodHandler(HashMap<String, Method> methods) {
		super(methods);
	}

	@Override
	public Result<IPCSuccessResponse, IPCFailureResponse> handleMessage(IIPCMessage _message) {
		if (!(_message instanceof AuthIPCMessage)) {
			return Result.Err(new IPCFailureResponse("UNAUTHORIZED"));
		}
		AuthIPCMessage message = (AuthIPCMessage) _message;
		if (!this.mappedMethods.containsKey(message.getMethod())) {
			return Result.Err(new IPCFailureResponse("Function not found"));
		}
		var method = this.mappedMethods.get(message.getMethod());
		var parameterTypes = method.getParameterTypes();
		Class<?>[] methodParameters = new Class<?>[parameterTypes.length - 1];
		for (int i = 1; i < parameterTypes.length; i++) {
			methodParameters[i - 1] = parameterTypes[i];
		}
		var user = message.getUser();
		if (parameterTypes.length == 0) {
			var result = Result
					.wrapping(() -> method.invoke(IPCListener.getTA(), user))
					.andThen(r -> IPCResponse.Success(r))
					.mapErr((x) -> new IPCFailureResponse("Invocation exception"));
			return result;
		}
		if (message.getParams() == null || message.getParams().size() != parameterTypes.length - 1) {
			return Result.Err(new IPCFailureResponse("Incorrect number of parameters"));
		}
		var mappedParams = mapParams(methodParameters, message.getParams());
		mappedParams.add(0, user);
		return Result.wrapping(() -> method.invoke(IPCListener.getTA(), mappedParams.toArray()))
				.andThen(r -> IPCResponse.Success(r))
				.mapErr(e -> new IPCFailureResponse(((InvocationTargetException)e).getTargetException().getMessage()));

	}
}
