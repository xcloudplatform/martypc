/*
    Marty PC Emulator 
    (C)2023 Daniel Balsom
    https://github.com/dbalsom/marty

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.


    egui::pit_viewer.rs

    Implements a viewer control for the Programmable Interval Timer.
    
    This viewer displays data regarding the Programmable Interval Timer's 
    3 channels, as well as displaying a graph of the timer output.

*/

use egui::*;
use egui::plot::{
    Line, 
    //Plot, 
    PlotPoints, 
    //PlotBounds
};

use crate::egui::*;
use crate::egui::color::*;
use crate::egui::constants::*;

use crate::devices::pit::PitDisplayState;
use crate::syntax_token::*;

#[allow (dead_code)]
pub struct PitViewerControl {

    pit_state: PitDisplayState,
    channel_vecs: [Vec<u8>; 3],
    channel_data: [PlotPoints; 3],
    channel_lines: [Line; 3]
}

impl PitViewerControl {

    pub fn new() -> Self {
        Self {
            pit_state: Default::default(),
            channel_vecs: [
                Vec::new(), Vec::new(), Vec::new()
            ],
            channel_data: [
                PlotPoints::new(Vec::new()),
                PlotPoints::new(Vec::new()),
                PlotPoints::new(Vec::new())
            ],
            channel_lines: [
                Line::new(PlotPoints::new(Vec::new())),
                Line::new(PlotPoints::new(Vec::new())),
                Line::new(PlotPoints::new(Vec::new()))
            ]
        }
    }

    pub fn draw(&mut self, ui: &mut egui::Ui, _events: &mut VecDeque<GuiEvent> ) {

        for (i, channel) in self.pit_state.iter().enumerate() {

            egui::CollapsingHeader::new(format!("Channel: {}", i))
            .default_open(true)
            .show(ui, |ui| {

                ui.horizontal(|ui| {
                    ui.set_min_width(PIT_VIEWER_WIDTH);
                    ui.group(|ui| {

                        ui.set_min_width(PIT_VIEWER_WIDTH);

                        egui::Grid::new(format!("pit_view{}", i))
                        .num_columns(2)
                        .spacing([40.0, 4.0])
                        .striped(true)

                        .show(ui, |ui| {
                        
                            for (key, value) in channel {

                                if let SyntaxToken::StateString(text, _, age) = value {
                                    ui.label(egui::RichText::new(*key).text_style(egui::TextStyle::Monospace));
                                    ui.label(
                                        egui::RichText::new(text)
                                            .text_style(egui::TextStyle::Monospace)
                                            .color(fade_c32(Color32::GRAY, STATUS_UPDATE_COLOR, 255-*age))
                                        );
                                    //ui.add(egui::TextEdit::singleline(&mut self.pit_state.c0_access_mode).font(egui::TextStyle::Monospace));
                                    ui.end_row();

                                    
                                }
                            }
                        });
                    });
                });

                /*
                Plot::new(format!("pit_plot{}", i))
                .view_aspect(2.0)
                .width(PIT_VIEWER_WIDTH - 10.0)
                .height(75.0)
                .allow_scroll(false)
                .allow_zoom(false)
                .show_x(true)
                .show_y(true)
                .show(ui, |ui| {

                    ui.set_plot_bounds(PlotBounds::from_min_max([0.0,0.0], [100.0,1.0]));

                    let points: PlotPoints = self.channel_vecs[i].iter().enumerate().map(|i| {
                        
                        let x = i.0 as f64;
            
                        // Convert u8 to f32
                        let y = if *i.1 == 1u8 { 1.0 } else { 0.0 };
            
                        [x, y]
                    }).collect();

                    ui.line(Line::new(points));
                });
                */
            });

        }  
    }

    pub fn update_state(&mut self, state: &PitDisplayState ) {


        let mut new_pit_state = state.clone();

        // Update state entry ages
        for (i, channel) in new_pit_state.iter_mut().enumerate() {
            for (key, value) in channel.iter_mut() {

                if let SyntaxToken::StateString(_txt, dirty, age) = value {
                    if *dirty {
                        *age = 0;
                    }
                    else if i < self.pit_state.len() {
                        if let Some(old_tok) = self.pit_state[i].get_mut(key) {
                            if let SyntaxToken::StateString(_,_,old_age) = old_tok {
                                *age = old_age.saturating_add(2);                            
                            }
                        }
                    }
                }
            }
        }

        self.pit_state = new_pit_state;
    }

    pub fn update_channel_data(&mut self, channel: usize, data: &[u8]) {

        self.channel_vecs[channel] = data.to_vec();



        //self.channel_data[channel] = points;
        //self.channel_lines[channel] = Line::new(points);
    }
    

}

