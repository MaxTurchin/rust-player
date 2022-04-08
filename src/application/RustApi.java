package application;

public class RustApi {
	private long player_ptr;
	private long queue_ptr;
	
	private static native long[] rInit();			
	private native void rPlayOrPause(long ptr);
	private native void rAddToQueue(long ptr, String fpath);

	static {
		System.loadLibrary("rust_player");
	}
	
	public RustApi() {
		long []ptrs  = rInit();		// Gets raw pointers for [player struct, queue struct]
		this.player_ptr = ptrs[0];	// Player struct raw pointer
		this.queue_ptr = ptrs[1];	// Queue struct raw pointer
	}
	
	public void playOrPause() {
		this.rPlayOrPause(player_ptr);
	}
	
	public void addToQueue(String fpath) {
		System.out.println("added " + fpath);
		this.rAddToQueue(this.queue_ptr, fpath);
	}
	
//	public void print_content() {
//		System.out.println(this.get_content(this.ptr));
//	}
	
}
