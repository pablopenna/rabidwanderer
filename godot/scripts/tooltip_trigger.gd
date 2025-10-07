extends Control



func _on_mouse_entered() -> void:
	GlobalSignals.show_tooltip.emit("meow")


func _on_mouse_exited() -> void:
	GlobalSignals.hide_tooltip.emit()
