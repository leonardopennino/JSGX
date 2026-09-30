package com.sicpa.javata;

import com.fasterxml.jackson.databind.JsonNode;

public class AuthIPCMessage extends IIPCMessage {
	private String method;
	private JsonNode params;
	private User user;

	private AuthIPCMessage() {
	}

	public String getMethod() {
		return method;
	}

	public JsonNode getParams() {
		return params;
	}

	public User getUser() {
		return user;
	}


}
