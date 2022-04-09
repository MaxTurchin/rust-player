package application;

public class RustApi {
	private static PlayerEvents events;
	private static long player_ptr;
	private static long queue_ptr;
	
	private static native long[] rInit(RustApi callback);
	private native static void rPlayOrPause(long ptr);
	private native static void rAddToQueue(long ptr, String fpath);

	static {
		System.loadLibrary("rust_player");
	}
	
	public static void initPlayer(PlayerEvents e) {
		long []ptrs  = rInit(new RustApi());  // Gets raw pointers for [player struct, queue struct]
		player_ptr = ptrs[0];			// Player struct raw pointer
		queue_ptr = ptrs[1];			// Queue struct raw pointer
		events = e;
	}
	
	public static void playOrPause() {
		rPlayOrPause(player_ptr);
	}
	
	public static void addToQueue(String fpath) {
		System.out.println("added " + fpath);
		rAddToQueue(queue_ptr, fpath);
	}
	
	public void updateNowPlaying(String title, String artist, String album) {
		events.updateNowPlaying(title, artist, album);
	}
}
