package com.sicpa.javata;

import java.util.Set;

import com.fasterxml.jackson.databind.ObjectMapper;

public record User(
		String uuid,
		Set<String> capabilities) {

	public static User FromRust(String json) {
		ObjectMapper mapper = new ObjectMapper();
		try {
			return mapper.readValue(json, User.class);
		} catch (Exception e) {
			e.printStackTrace();
			return null;
		}
	}
}
