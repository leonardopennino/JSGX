package com.sicpa.trustedapplication;
import java.util.Arrays;
import java.util.concurrent.ConcurrentHashMap;
import java.util.stream.Collectors;

import com.sicpa.javata.User;
import com.sicpa.jsgxannotations.AuthenticatedSecureFunction;
import com.sicpa.jsgxannotations.SecureFunction;
import com.sicpa.jsgxannotations.TrustedApplication;

@TrustedApplication
public class TA {
	private ConcurrentHashMap<String, String[]> blackList;
	private final static int NGovernments = 2;
	private String[] computedBlacklist = null;

	public TA() {
		blackList = new ConcurrentHashMap<>();
	}

	@AuthenticatedSecureFunction
	public boolean AddGovernmentBlacklist(User government, String[] blackList) {
		if (!government.capabilities().contains("ADD")) {
			return false;
		}
		if (this.blackList.contains(government.uuid())) {
			return false;
		}
		this.blackList.put(government.uuid(), blackList);
		return true;
	}

	@AuthenticatedSecureFunction
	public String[] getOutput(User government) {
		if (!government.capabilities().contains("READ")) {
			throw new RuntimeException("Government is not allowed to read the content");
			//return null;
		}
		if (computedBlacklist != null) {
			return computedBlacklist;
		} else if (this.blackList.size() == NGovernments) {
			computeBlackList();
			return computedBlacklist;
		} else {
			return null;
		}
	}
	@SecureFunction
	public boolean isHealthy() {
	  return true;
	}

	private void computeBlackList() {
		var values = blackList.values().stream().flatMap(b -> Arrays.stream(b))
				.collect(Collectors.groupingBy(String::toString, Collectors.counting()));
		String[] filtered = values.keySet().stream().filter(k -> values.get(k) == NGovernments).toArray(String[]::new);
		this.computedBlacklist = filtered;
	}

}
