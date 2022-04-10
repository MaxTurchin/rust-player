package application;

import java.net.URL;
import java.util.ResourceBundle;

import javafx.application.Platform;
import javafx.beans.value.ChangeListener;
import javafx.beans.value.ObservableValue;
import javafx.event.ActionEvent;
import javafx.event.EventHandler;
import javafx.fxml.FXML;
import javafx.fxml.Initializable;
import javafx.scene.control.Button;
import javafx.scene.control.Label;
import javafx.scene.control.ListView;
import javafx.scene.input.MouseEvent;

public class PlayerEvents implements Initializable{
	@FXML
	private Button playPause;
	private Button playPrevious;
	private Button playNext;
	
	@FXML
	private Label titleLabel;
	@FXML
	private Label albumLabel;
	@FXML
	private Label artistLabel;

	@FXML
	private ListView <String> songList;
	
	private String[] songs = {"C:\\Users\\Max\\Music\\FET - 5325\\FET - 5325 - 01 Particle Accelerator.mp3", "C:\\Users\\Max\\Music\\third ever.mp3"};
	
	
	@Override
	public void initialize(URL arg0, ResourceBundle arg1) {
		RustApi.initPlayer(this);
		titleLabel.setText("");
		albumLabel.setText("");
		artistLabel.setText("");

		songList.getItems().addAll(songs);
		songList.setOnMouseClicked(new EventHandler<MouseEvent>() {

			@Override
			public void handle(MouseEvent click) {
				if (click.getClickCount() == 2) {
		           String song = songList.getSelectionModel().getSelectedItem();
		           if (song != null) {
		        	   RustApi.addToQueue(song);
		           }
				}
			}
		});
	}
	
	
	public void playPause(ActionEvent e) {
		System.out.println("play pause");
		RustApi.playOrPause();

		if (playPause.getText() == "Play") {
			playPause.setText("Pause");
		} else {
			playPause.setText("Play");
		}
	}
	
	public void previous(ActionEvent e) {
		System.out.println("play previous");
		RustApi.playPrevious();
	}
	
	public void next(ActionEvent e) {
		System.out.println("play next");
		RustApi.playNext();
	}

	
	public void updateNowPlaying(String title, String artist, String album) {
		Platform.runLater(new Runnable() {
			@Override public void run() {
				albumLabel.setText(album);
				artistLabel.setText(artist);
				titleLabel.setText(title);
		    }
		});
	}
}
