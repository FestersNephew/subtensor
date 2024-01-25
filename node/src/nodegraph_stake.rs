unsafe fn paint_nodegraph_stake(ui: &mut egui::Ui)
{
    for (netuid, stake) in pallet_subtensor::get_stake_map().iter().enumerate()
    {
        ui.label(
            format!(
                "netuid: {:?} stake: {:?}", 
                netuid,
                stake
            )
        );
    }
}