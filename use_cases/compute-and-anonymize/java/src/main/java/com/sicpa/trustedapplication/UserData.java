package com.sicpa.trustedapplication;

import java.util.List;

public record UserData(String username, List<Transaction> transactions) {}
