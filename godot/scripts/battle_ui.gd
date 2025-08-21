# DEPRECATED - migrated to Rust (BattleUi)
extends Control

@export var subviewport: SubViewport
@export var battle_engine: BattleEngine

func _input(event: InputEvent) -> void:
	if event.is_action_pressed("ui_text_indent"):
		self.visible = !self.visible
		if self.visible:
			subviewport.process_mode = Node.PROCESS_MODE_INHERIT
		else:
			subviewport.process_mode = Node.PROCESS_MODE_DISABLED
			
	if event.is_action_pressed("ui_accept") and self.visible:
		print("starting...")
		battle_engine.start_battle()
