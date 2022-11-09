extends Node

func _ready():
	print("hello worl")


func _on_Button_pressed():
	var Tester = load("res://scenes/Node.gdns")
	var tester = Tester.new()
	tester.say_hello()
