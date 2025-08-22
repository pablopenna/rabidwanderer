extends Control

func _ready():
	GlobalSignals.player_died.connect(func (): self.visible = true)

func _on_button_pressed() -> void:
	print("Ni hao")
