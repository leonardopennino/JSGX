package com.sicpa.javata;

import com.fasterxml.jackson.databind.JsonNode;

public class IPCMessage extends IIPCMessage {
	private String method;
	private JsonNode params;

	private IPCMessage() { }

	public String getMethod() {
		return method;
	}

	public JsonNode getParams() {
		return params;
	}
}
