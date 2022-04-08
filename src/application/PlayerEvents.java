package application;

import java.net.URL;
import java.util.ResourceBundle;

import javafx.beans.value.ChangeListener;
import javafx.beans.value.ObservableValue;
import javafx.event.ActionEvent;
import javafx.event.EventHandler;
import javafx.fxml.FXML;
import javafx.fxml.Initializable;
import javafx.scene.control.Button;
import javafx.scene.control.ListView;
import javafx.scene.input.MouseEvent;

public class PlayerEvents implements Initializable{
	@FXML
	private Button playPause;
	private Button playPrevious;
	private Button playNext;
	
	@FXML
	private ListView <String> songList;
	
	private RustApi rApi = new RustApi();
	private String[] songs = {"C:\\Users\\Max\\Music\\third ever.mp3", "C:\\Users\\Max\\Music\\first ever.mp3"};
	
	
	@Override
	public void initialize(URL arg0, ResourceBundle arg1) {
		songList.getItems().addAll(songs);
		songList.setOnMouseClicked(new EventHandler<MouseEvent>() {

			@Override
			public void handle(MouseEvent click) {
				if (click.getClickCount() == 2) {
			           String song = songList.getSelectionModel().getSelectedItem();
			           if (song != null) {
			        	   rApi.addToQueue(song);
			           }
				}
			}
		});
	}
	
	
	public void playPause(ActionEvent e) {
		System.out.println("play pause");
		this.rApi.playOrPause();
		
	}
	
	public void previous(ActionEvent e) {
//		this.rApi.addToQueue();
		System.out.println("play previous");
	}
	
	public void next(ActionEvent e) {
		System.out.println("play next");
	}
}
