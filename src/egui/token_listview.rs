
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


    egui::token_listview.rs

    Implements a listview control based on syntax tokens.

    The control is a virtual window that can be scrolled over a specified
    virtual size. Contents are provided to the listview control for the 
    visible window. Contents are constructed from vectors of syntax tokens
    to enable color syntax highlighting, hover tooltips and other features.

*/
use std::mem::discriminant;

use egui::*;
use crate::egui::*;
use crate::egui::color::*;
use crate::egui::constants::*;
use crate::syntax_token::*;


pub struct TokenListView {

    pub row: usize,
    pub previous_row: usize,
    pub visible_rows: usize,
    pub max_rows: usize,
    pub contents: Vec<Vec<SyntaxToken>>,
    pub visible_rect: Rect,

    pub l_margin: f32,
    pub t_margin: f32,

    hover_text: String,
}

impl TokenListView {

    pub fn new() -> Self {
        Self {
            row: 0,
            previous_row: 0,
            visible_rows: 16,
            max_rows: 0,
            contents: Vec::new(),
            visible_rect: Rect::NOTHING,

            l_margin: 5.0,
            t_margin: 3.0,

            hover_text: String::new()
        }
    }

    pub fn set_visible(&mut self, size: usize) {
        self.visible_rows = size;
    }

    pub fn set_capacity(&mut self, size: usize) {
        self.max_rows = size;
    }

    pub fn set_contents(&mut self, mut contents: Vec<Vec<SyntaxToken>>) {

        if self.contents.len() != contents.len() {
            // Size of contents is changing. Assume these are all new bytes.

            for row in &mut contents {
                for mut token in row {

                    match &mut token {
                        SyntaxToken::MemoryByteHexValue(_,_,_,_,new_age) => {
                            *new_age = TOKEN_MAX_AGE;
                        }
                        SyntaxToken::MemoryByteAsciiValue(_,_,_,new_age) => {
                            *new_age = TOKEN_MAX_AGE
                        }
                        _ => {}
                    }
                }
            }
            self.contents = contents;
            return
        }

        // Age incoming SyntaxTokens.
        for row_it in contents.iter_mut().zip(self.contents.iter_mut()) {

            for token_it in row_it.0.iter_mut().zip(row_it.1.iter()) {

                let (new, old) = token_it;

                if discriminant(new) == discriminant(old) {
                    // Token types match

                    match (new, old) {

                        (SyntaxToken::MemoryByteHexValue(new_addr,new_val,_,_,new_age), SyntaxToken::MemoryByteHexValue(old_addr,old_val,_,_,old_age)) => {
                            if old_addr == new_addr {
                                // This is the same byte as before. Compare values.
                                if old_val == new_val {
                                    // Byte hasn't changed, so increment age.
                                    *new_age = old_age.saturating_add(2);    
                                }
                            }
                            else {
                                // Different byte address in this position. Set age to maximum so it doesn't flash.
                                *new_age = 255;
                            }
                        }
                        (SyntaxToken::MemoryByteAsciiValue(new_addr,new_val,_,new_age), SyntaxToken::MemoryByteAsciiValue(old_addr,old_val,_,old_age)) => {
                            if old_addr == new_addr {
                                // This is the same byte as before. Compare values.
                                if old_val == new_val {
                                    // Byte hasn't changed, so increment age.
                                    *new_age = old_age.saturating_add(2);    
                                }
                            }
                            else {
                                // Different byte address in this position. Set age to maximum so it doesn't flash.
                                *new_age = 255;
                            }                            
                        }             
                        _ => {}
                    }
                }
            }
        }

        self.contents = contents;
    }

    pub fn set_hover_text(&mut self, text: String) {
        self.hover_text = text;
    }

    pub fn measure_token(&self, ui: &mut Ui, token: &SyntaxToken, fontid: FontId ) -> Rect {

        let old_clip_rect = ui.clip_rect();
        //let old_cursor = ui.cursor();
        ui.set_clip_rect(Rect::NOTHING);
        let r = ui.painter().text(
            egui::pos2(0.0, 0.0),
            egui::Align2::LEFT_TOP,
            match token {
                SyntaxToken::MemoryByteHexValue(_, _, s, _, _) => s.clone(),
                _ => "0".to_string()
            },
            fontid,
            Color32::LIGHT_GRAY,
        );
        ui.set_clip_rect(old_clip_rect);
        //ui.set_cursor(old_cursor);
        r
    }

    pub fn draw(&mut self, ui: &mut egui::Ui, events: &mut VecDeque<GuiEvent>, new_row: &mut usize) {

        let font_id = egui::TextStyle::Monospace.resolve(ui.style());
        let row_height = ui.fonts().row_height(&font_id) + ui.spacing().item_spacing.y;
        let num_rows = self.max_rows;
        let show_rows = self.visible_rows;

        ui.set_height(row_height * show_rows as f32);

        let mut used_rect = egui::Rect::NOTHING;
        
        // Draw background rect
        ui.painter().rect_filled(
            ui.max_rect(),
            
            egui::Rounding::default(),
            egui::Color32::BLACK
        );

        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show_viewport(ui, |ui, viewport| {

                ui.set_height(row_height * num_rows as f32);
                
                //log::debug!("viewport.min.y: {}", viewport.min.y);
                let mut first_item = (viewport.min.y / row_height).floor().at_least(0.0)as usize;
                let last_item = (viewport.max.y / row_height).ceil() as usize + 1;
                let last_item = last_item.at_most(num_rows - show_rows);

                if first_item > last_item {
                    first_item = last_item;
                }

                self.row = first_item;

                if self.row != self.previous_row {
                    // View was scrolled, update address
                    
                    *new_row = self.row & !0x0F;
                    self.previous_row = self.row;
                    
                    events.push_back(GuiEvent::MemoryUpdate);
                }
                
                //let start_y = ui.min_rect().top() + (first_item as f32) * row_height;
                let start_y = viewport.min.y + ui.min_rect().top();

                // Constrain visible rows if we don't have enough rows in contents
                let show_rows = usize::min(show_rows, self.contents.len());
                
                // Measure the size of a byte token label.
                let label_rect = 
                    self.measure_token(
                        ui, 
                        &SyntaxToken::MemoryByteHexValue(0, 0, "00".to_string(), false, 0),
                        font_id.clone()
                    );

                let l_bracket = "[".to_string();
                let r_bracket = "]".to_string();
                let colon = ":".to_string();
                let comma = ",".to_string();
                let plus = "+".to_string();
                let null = "[missing token!]".to_string();

                for (i, row) in self.contents[0..show_rows].iter().enumerate() {
                    let x = ui.min_rect().left() + self.l_margin;
                    let y = start_y + ((i as f32) * row_height) + self.t_margin;

                    let mut token_x = x;

                    let mut column_select = 32; // Initial value out of range to not highlight anything
                    for (j, token) in row.iter().enumerate() {

                        let mut text_rect;

                        let drawn;
                        match token {
                            SyntaxToken::MemoryAddressFlat(_addr, s) => {
                                text_rect = ui.painter().text(
                                    egui::pos2(token_x, y),
                                    egui::Align2::LEFT_TOP,
                                    s,
                                    font_id.clone(),
                                    Color32::LIGHT_GRAY,
                                );
                                token_x = text_rect.max.x + 10.0;
                                used_rect = used_rect.union(text_rect);
                                drawn = true;
                            }
                            SyntaxToken::MemoryByteHexValue(addr, _, s, cursor, age) => {

                                if ui.put(
                                    Rect {
                                        min: egui::pos2(token_x, y), 
                                        max: egui::pos2(token_x + label_rect.max.x + 1.0, y + label_rect.max.y)
                                    },
                                    egui::Label::new(
                                        egui::RichText::new(s)
                                            .text_style(egui::TextStyle::Monospace)
                                            .color(fade_c32(Color32::GRAY, Color32::from_rgb(0, 255, 255), 255-*age))
                                        )
                                )
                                .on_hover_text(format!("{}", self.hover_text))
                                .hovered() {
                                    column_select = j;
                                    events.push_back(GuiEvent::TokenHover(*addr as usize));
                                }

                                if *cursor {
                                    ui.painter().rect(
                                        Rect {
                                            min: egui::pos2(token_x, y), 
                                            max: egui::pos2(token_x + label_rect.max.x + 1.0, y + label_rect.max.y)
                                        },
                                        egui::Rounding::none(),
                                        Color32::TRANSPARENT,
                                        egui::Stroke::new(1.0, Color32::WHITE)
                                    );                                    
                                }

                                token_x += label_rect.max.x + 7.0;
                                drawn = true;
                                /*
                                text_rect = ui.painter().text(
                                    egui::pos2(token_x, y),
                                    egui::Align2::LEFT_TOP,
                                    s,
                                    font_id.clone(),
                                    Color32::LIGHT_BLUE,
                                );
                                token_x = text_rect.max.x + 7.0;
                                used_rect = used_rect.union(text_rect);
                                */
                            }
                            SyntaxToken::MemoryByteAsciiValue(_addr, _, s, age) => {
                                text_rect = ui.painter().text(
                                    egui::pos2(token_x, y),
                                    egui::Align2::LEFT_TOP,
                                    s,
                                    font_id.clone(),
                                    fade_c32(Color32::LIGHT_GRAY, Color32::from_rgb(0, 255, 255), 255-*age),
                                );

                                // If previous hex byte was hovered, show a rectangle around this ascii byte
                                // TODO: Rather than rely on hex bytes directly preceding the ascii bytes, 
                                // use an 'index' field in the enum?
                                if (j - 16) == column_select {
                                    ui.painter().rect(
                                        text_rect.expand(2.0),
                                        egui::Rounding::none(),
                                        Color32::TRANSPARENT,
                                        egui::Stroke::new(1.0, COLOR32_CYAN)
                                    );
                                }

                                token_x = text_rect.max.x + 2.0;
                                used_rect = used_rect.union(text_rect);
                                drawn = true;
                            }
                            SyntaxToken::Mnemonic(s) => {
                                text_rect = ui.painter().text(
                                    egui::pos2(token_x, y),
                                    egui::Align2::LEFT_TOP,
                                    s,
                                    font_id.clone(),
                                    Color32::from_rgb(128, 255, 158),
                                );
                                token_x = text_rect.min.x + 45.0;
                                used_rect = used_rect.union(text_rect);
                                drawn = true;
                            }
                            _ => {
                                drawn = false;
                            }                      
                        }

                        if !drawn { 
                            
                            let (token_color, token_text, token_padding) = match token {
                                SyntaxToken::MemoryAddressSeg16(_,_,s) => {
                                    (Color32::LIGHT_GRAY, s, 10.0) 
                                }
                                SyntaxToken::InstructionBytes(s) => {
                                    (Color32::from_rgb(6, 152, 255), s, 1.0)
                                }
                                SyntaxToken::Prefix(s) => {
                                    (Color32::from_rgb(116, 228, 227), s, 2.0)
                                }
                                SyntaxToken::Register(s) => {
                                    (Color32::from_rgb(245, 138, 52), s, 1.0)
                                }
                                SyntaxToken::OpenBracket => {
                                    (Color32::from_rgb(228, 214, 116), &l_bracket, 1.0)
                                }
                                SyntaxToken::CloseBracket => {
                                    (Color32::from_rgb(228, 214, 116), &r_bracket, 2.0)
                                }
                                SyntaxToken::Colon => {
                                    (Color32::LIGHT_GRAY, &colon, 1.0) 
                                }
                                SyntaxToken::Comma => {
                                    (Color32::LIGHT_GRAY, &comma, 6.0) 
                                }
                                SyntaxToken::PlusSign => {
                                    (Color32::LIGHT_GRAY, &plus, 1.0) 
                                }                                                              
                                SyntaxToken::Displacement(s) | SyntaxToken::HexValue(s) => {
                                    (Color32::from_rgb(96, 200, 210), s, 2.0)
                                }
                                SyntaxToken::Segment(s) => {
                                    (Color32::from_rgb(245, 138, 52), s, 1.0)
                                }
                                SyntaxToken::Text(s) => {
                                    (Color32::LIGHT_GRAY, s, 2.0) 
                                }
                                SyntaxToken::ErrorString(s) => {
                                    (Color32::RED, s, 2.0) 
                                }                                                                                                                                 
                                _ => (Color32::WHITE, &null, 2.0)

                            };

                            text_rect = ui.painter().text(
                                egui::pos2(token_x, y),
                                egui::Align2::LEFT_TOP,
                                token_text,
                                font_id.clone(),
                                token_color,
                            );
                            token_x = text_rect.max.x + token_padding;
                            used_rect = used_rect.union(text_rect); 
                        }
                    }
                }

                //egui::TextEdit::multiline(&mut format!("hi!"))
                //    .font(egui::TextStyle::Monospace);

                ui.allocate_rect(used_rect, egui::Sense::hover());
            });
    }
}