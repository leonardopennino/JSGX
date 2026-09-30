package com.sicpa.trustedapplication;

import java.util.List;
import java.util.concurrent.ConcurrentHashMap;

import com.sicpa.javata.AuthenticatedSecureFunction;
import com.sicpa.javata.User;

public class TrustedApplication {
	private ConcurrentHashMap<String, List<UserData>> data = new ConcurrentHashMap<>();
	// private static final int NREQUIREDUSERS = 2;

	@AuthenticatedSecureFunction
	public void uploadData(User user, List<UserData> userData) {
		if (!user.capabilities().contains("UPLOAD")) {
			throw new RuntimeException("The user does not have the right to call this function");
		}
		if (data.contains(user.uuid())) {
			throw new RuntimeException("The user has already uploaded data");
		}
		data.put(user.uuid(), userData);
	}

  /*
	@AuthenticatedSecureFunction
	public long getTransactionsOfItem(User user, String item) {
		if (data.size() != NREQUIREDUSERS) {
			throw new RuntimeException("Not all users have uploaded data");
		}
		return data.values().stream()
				.flatMap(lst -> lst.stream().flatMap(data -> data.transactions().stream().filter(t -> t.item() == item)))
				.count();
	}

	@SecureFunction
	public long getTotalTransactions() {
		if (data.size() != NREQUIREDUSERS) {
			throw new RuntimeException("Not all users have uploaded data");
		}
		return data.values().stream()
				.flatMap(lst -> lst.stream().flatMap(data -> data.transactions().stream()))
				.count();
	}
	*/
}
