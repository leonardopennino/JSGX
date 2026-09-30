package com.sicpa.javata;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;

public abstract class IIPCMessage {
	public static IIPCMessage ReadFromRust(String json) {
		try {
			JsonNode node = new ObjectMapper().readTree(json);
			Class<? extends IIPCMessage> c;

			if (node.get("user") != null && node.get("user").isObject()) {
				c = AuthIPCMessage.class;
			} else {
				c = IPCMessage.class;
			}

			ObjectMapper mapper = new ObjectMapper();
			try {
				return mapper.readValue(json, c);
			} catch (Exception e) {
				e.printStackTrace();
				return null;
			}
		} catch (Exception e) {
			return null;
		}
	}
	public abstract String getMethod();
	public abstract JsonNode getParams();
}
