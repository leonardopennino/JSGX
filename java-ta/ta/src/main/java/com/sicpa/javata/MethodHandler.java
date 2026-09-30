package com.sicpa.javata;

import java.lang.reflect.Method;
import java.util.ArrayList;
import java.util.HashMap;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.sicpa.javata.IPCResponse.IPCFailureResponse;
import com.sicpa.javata.IPCResponse.IPCSuccessResponse;

public abstract class MethodHandler {
	protected HashMap<String, Method> mappedMethods;

	public MethodHandler(HashMap<String, Method> methods) {
		this.mappedMethods = methods;
	}

	public abstract Result<IPCSuccessResponse, IPCFailureResponse> handleMessage(IIPCMessage message);

	protected ArrayList<Object> mapParams(Class<?>[] methodParams, JsonNode IPCParams) {
		var mappedParams = new ArrayList<>(methodParams.length);
		for (int i = 0; i < methodParams.length; i++) {
			var mapped = mapParam(methodParams[i], IPCParams.get(i));
			mappedParams.add(i,mapped);
		}
		return mappedParams;
	}

	private <T> Object mapParam(Class<T> methodParam, JsonNode IPCParam) {
		ObjectMapper mapper = new ObjectMapper();
		return mapper.convertValue(IPCParam, methodParam);
	}
}
