extends PanelContainer

# PopupPanel - https://www.youtube.com/watch?v=Q8oee9tRMSc
# Panel + RichText Label - https://www.youtube.com/watch?v=6OyPgL2Elpw

const OFFSET: Vector2 = Vector2.ONE * 5.0

func _input(event: InputEvent) -> void:
	if visible and event is InputEventMouseMotion:
		global_position = get_global_mouse_position() + OFFSET
		if is_out_of_screen(): 
			adjust()

func is_out_of_screen() -> bool:
	var view_rect: Rect2 = get_viewport_rect()
	var tooltip_rect: Rect2 = self.get_global_rect()
	return not view_rect.encloses(tooltip_rect)

func adjust() -> void:
	var max_pos: Vector2 = get_viewport_rect().size - self.get_rect().size
	var min_pos: Vector2 = Vector2.ZERO
	self.global_position.x = clamp(self.global_position.x, min_pos.x, max_pos.x)
	self.global_position.y = clamp(self.global_position.y, min_pos.y, max_pos.y)
