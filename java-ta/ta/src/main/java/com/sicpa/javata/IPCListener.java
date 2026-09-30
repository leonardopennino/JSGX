package com.sicpa.javata;


import com.sicpa.jsgxannotations.AuthenticatedSecureFunction;
import com.sicpa.jsgxannotations.SecureFunction;

import java.io.IOException;
import java.lang.reflect.Method;
import java.net.SocketAddress;
import java.net.StandardProtocolFamily;
import java.net.UnixDomainSocketAddress;
import java.nio.channels.ServerSocketChannel;
import java.nio.channels.SocketChannel;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.*;
import java.util.concurrent.Executor;
import java.util.concurrent.Executors;


public class IPCListener  {
    private static Object TA;
    private HashMap<String, Method> mapMethods = new HashMap<>();
    private HashMap<String, Method> authMapMethods = new HashMap<>();
    private static final int NTHREADS = 12;
    private static final Executor exec = Executors.newFixedThreadPool(NTHREADS);
	private static final String SOCKET_PATH = "/tmp/socket";

    public IPCListener(Object ta) {
        TA = ta;
        var methods = Arrays.stream(TA.getClass().getMethods()).filter(m -> m.isAnnotationPresent(SecureFunction.class)).toArray(Method[]::new);
		var authMethods =Arrays.stream(TA.getClass().getMethods()).filter(m -> m.isAnnotationPresent(AuthenticatedSecureFunction.class)).toArray(Method[]::new);
		Arrays.stream(methods).forEach(method -> this.mapMethods.put(method.getName(), method));
		Arrays.stream(authMethods).forEach(method -> this.authMapMethods.put(method.getName(), method));
    }

    public static Object getTA(){
        return TA;
    }

    public void run() {
        ServerSocketChannel serverSocketChannel;
        try {
            Files.deleteIfExists(Path.of(Paths.get(SOCKET_PATH).toUri()));
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
        SocketAddress socketAddress = UnixDomainSocketAddress.of(SOCKET_PATH);
        try {
            serverSocketChannel = ServerSocketChannel.open(StandardProtocolFamily.UNIX);
            serverSocketChannel.bind(socketAddress);
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
        try {
		  System.out.println("Opened socket at address " + serverSocketChannel.getLocalAddress());
            while (true) {
                final SocketChannel channel = serverSocketChannel.accept();
				final SecureMethodHandler methodHandler = new SecureMethodHandler(mapMethods);
				final AuthSecureMethodHandler authMethodHandler = new AuthSecureMethodHandler(authMapMethods);
                IPCExecutor executor = new IPCExecutor(channel, methodHandler, authMethodHandler );
                exec.execute(executor);
            }
        }
        catch (Exception e) {

        }
    }
}
