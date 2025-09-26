extends PanelContainer

# PopupPanel - https://www.youtube.com/watch?v=Q8oee9tRMSc
# Panel + RichText Label - https://www.youtube.com/watch?v=6OyPgL2Elpw

const OFFSET: Vector2 = Vector2.ONE * 20.0

func _input(event: InputEvent) -> void:
	if visible and event is InputEventMouseMotion:
		global_position = get_global_mouse_position() + OFFSET
		
