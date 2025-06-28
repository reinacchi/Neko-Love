package main

import (
	"neko-love/routes"

	"github.com/gofiber/fiber/v2"
)

// main initializes and starts the Fiber web server, sets up static file serving from the "./assets" directory,
// defines API route groups under "/api/v4", and registers application routes using the SetupRoutes function.
// The server listens on port 3030.
func main() {
	app := fiber.New()

	app.Static("/static", "./assets")

	api := app.Group("/api")
	v4 := api.Group("/v4")

	routes.SetupRoutes(v4)

	app.Listen(":3030")
}