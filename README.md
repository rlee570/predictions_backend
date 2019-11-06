# zuehlke_predictions_backend
Backend for the predictions platform

# To build and run
You need to install Rust and then cargo run to run the server.
Also requires postgressql now includes an init.sql to run against your postgresql.

# Alternatively
Use docker to build. Change to the directory in terminal to the project
then run:

```docker build -t zuehlke_predictions_backend .```

Then to run the server and postgres database together.

```docker run -it -p 8000:8000 -p 5432:5432 zuehlke_predictions_backend```

To run in the background i.e to not consume a terminal tab with the logs.

```docker run -it -d -p 8000:8000 -p 5432:5432 zuehlke_predictions_backend```
