# fawkes-api
Api wrapper for [fawkes](https://sandlab.cs.uchicago.edu/fawkes).
## What is fawkes?
Fawkes is a programm which uses ai to make images of peoples faces useless for building & training facial recognition AIs by poisoning their dataset,
this is done by running photos through fawkes before uploading them to the internet. When the "cleaned" images are then taken by someone and used to train a facial recognition model of your face,
the model will be poisoned and unable to actually detect your face if encountered in real life.
## What is the purpose of this repo?
This repo provides a api wrapper for the fawkes binary allowing you to run it on a more capable machine (you home pc or similar) while still having acess to its usefull features.


Once started the docker container (recommended) or the binary will run on port 8000 and allow calls to the api from there.

## Compile instructions:
### With docker:
  - Clone this repository
  - Enter the downloaded project
  - Run: "docker build dockerfile"
After build is complete you are left with a docker image you can run (don't forget to expose port 8000)

### Without docker:
  - Make sure you have the rust toolchain installed
  - Clone this repository
  - Enter the downloaded project
  - Run: "cargo build --release" (run without release for dev version with more logs)
The resulting binary can be run anywhere but it has to have following files/folders along side it:
  - directory called uploads
  - unzipped fawkes binary available [here](http://sandlab.cs.uchicago.edu/fawkes/) called protection

I strongly reccomend using the docker version since it takes care of all of this during creation.

## Api documentation:
### address:port/Upload
  Allows uploading images (jpeg or png) and returns image id
### address:port/download/<image_id>
  Allows you to download the finished or "clened" image once it's complete
### address:port/query/<image_id>
  Lets you check the status of your uploaded image and returns 
  - READY if the file is ready to download
  - NotReady if the file is still being processed
  - NotFound if the file is not found (wrong image id for example)
### address:port/health
  Simply returns Ok (code 200) and can be used to check if api is running
