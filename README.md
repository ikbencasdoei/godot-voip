# godot-voip-demo
  
## Introduction
Since there are basically no sources on how to send microphone input over network and microphone input has only been added recently in godot, I just decided to experiment myself. What I created is a very rough implementation but works well enough. I thought this was worth sharing so i decided to create this demo.

## How to setup for yourself
>There are some things I first overlooked so please take a look:
1. Enable audio input for your project (Project Settings -> Audio -> Enable Audio Input).
2. Create a new audio bus called Record.
3. Add the record effect and mute the audio bus.
4. Create an AudioStreamPlayer somewhere in your scene and add as stream a AudioStreamMicrophone to it.
5. Also enable Autoplay and set the output bus to the Record bus you created earlier.

Now you are able to record your microphone and use it to transmit it to an another AudioStreamPlayer over your server with help of the voip.gd script.
