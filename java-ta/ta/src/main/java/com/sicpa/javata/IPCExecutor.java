package com.sicpa.javata;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.sicpa.javata.IPCResponse.IPCFailureResponse;
import com.sicpa.javata.Result.Err;
import com.sicpa.javata.Result.Ok;

import java.io.IOException;
import java.nio.ByteBuffer;
import java.nio.channels.SocketChannel;
import java.util.Optional;

public class IPCExecutor implements Runnable {
	private final SocketChannel socket;
	private SecureMethodHandler methodHandler;
	private AuthSecureMethodHandler authMethodHandler;

	public IPCExecutor(SocketChannel socket, SecureMethodHandler methodHandler, AuthSecureMethodHandler authMethodHandler) {
		this.socket = socket;
		this.methodHandler = methodHandler;
		this.authMethodHandler = authMethodHandler;
	}

	@Override
	public void run() {
		try {
			while (true) {
				var socketMessage = readSocketMessage(socket);
				if (socketMessage.isEmpty()) {
					return;
				}
				var json = socketMessage.get();
				try {
					IIPCMessage message = IIPCMessage.ReadFromRust(json);
					if (message != null && message.getMethod() != null) {
						var handler = message instanceof AuthIPCMessage ? authMethodHandler : methodHandler;
						var result = handler.handleMessage(message);
						switch (result) {
							case Ok<? extends IPCResponse, ?> ok -> {
								var res = new ObjectMapper().writeValueAsString(ok.getValue());
								System.out.println("OK " + res);
								socket.write(ByteBuffer.wrap(res.getBytes()));
							}
							case Err<?, IPCFailureResponse> e -> {
								System.out.println("ERR " + e.getError());
								var res = new ObjectMapper().writeValueAsString(e.getError());
								socket.write(ByteBuffer.wrap(res.getBytes()));
							}
						}
					}
				} catch (Exception e) {
					e.printStackTrace();
					socket.write(ByteBuffer.wrap("\n".getBytes()));
				}
			}
		} catch (IOException e) {
			System.out.println(e);
			throw new RuntimeException(e);
		}
	}

	private Optional<String> readSocketMessage(SocketChannel channel) throws IOException {
		ByteBuffer buffer = ByteBuffer.allocate(1024);
		int bytesRead = channel.read(buffer);
		if (bytesRead < 0)
			return Optional.empty();
		byte[] bytes = new byte[bytesRead];
		buffer.flip();
		buffer.get(bytes);
		String message = new String(bytes);
		return Optional.of(message);
	}

}
