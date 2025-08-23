extends Control

func _ready():
	GlobalSignals.game_over.connect(func (): self.visible = true)

func _on_button_pressed() -> void:
	print("Ni hao")
