use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};

#[acmd_script( agent = "kamui", script = "game_attacklw3", category = ACMD_GAME )]
unsafe fn kamui_dtilt(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 5.0, 6.5);
    }
    macros::FT_MOTION_RATE(fighter, 0.5);
    frame(fighter.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(fighter, 0.8);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 85, 55, 0, 70, 2.8, 0.0, 4.5, 1.0, Some(0.0), Some(4.0), Some(3.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 78, 55, 0, 70, 2.8, 0.0, 4.5, 1.0, Some(0.0), Some(3.0), Some(11.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "kamui", script = "effect_attacklw3", category = ACMD_EFFECT )]
unsafe fn kamui_dtilt_fx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
		kamui_dtilt,
		kamui_dtilt_fx,
    );
}
