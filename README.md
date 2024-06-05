# Test Project Web Server for Serving Videos

This project is a simple web server that serves video files stored in a specified directory.

## Requirements
- rust and cargo

## Running the Project

Follow the instructions below to run the web server:

1. **Choose a directory to store the video files.**
   For example, use the directory `/Users/user/Downloads`.

2. **Place a video file in the chosen directory.**
   For example, save a file named `SampleVideo_1280x720_2mb.mp4`.

3. **Run the Rocket web server, specifying the `DATA_DIR` environment variable with your directory.**

   ```sh
   DATA_DIR=/Users/user/Downloads cargo run
   ```

4. **Open the video file in a browser.**
Navigate to http://localhost:8000/SampleVideo_1280x720_2mb.mp4 to view your video.