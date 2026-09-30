package com.sicpa.trustedapplication;
import com.sicpa.javata.*;

/**
 * Hello world!
 */
public class App {
    public static void main(String[] args) {
	  TrustedApplication ta = new TrustedApplication();
	  IPCListener listener = new IPCListener(ta);
	  listener.run();
    }
}
