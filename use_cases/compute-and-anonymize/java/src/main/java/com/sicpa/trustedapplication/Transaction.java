package com.sicpa.trustedapplication;

import java.util.Date;

public record Transaction(Date date, String item, int quantity) {
}
