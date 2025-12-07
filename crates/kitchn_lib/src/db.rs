use crate::ingredient::Ingredient;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct Pantry {
    path: PathBuf,
    ingredients: HashMap<String, Ingredient>,
}

impl Pantry {
    pub fn load(path: &Path) -> Result<Self> {
        let mut db = Pantry {
            path: path.to_path_buf(),
            ingredients: HashMap::new(),
        };

                if path.exists() {
            // Check if file is empty (e.g. newly created by NamedTempFile or touch)
            let len = fs::metadata(path).map(|m| m.len()).unwrap_or(0);
            if len > 0 {
                let file = File::open(path).context("Failed to open pantry database")?;
                let mut reader = BufReader::new(file);
                
                // Using bincode 2.0 serde integration
                let data: HashMap<String, Ingredient> = bincode::serde::decode_from_std_read(
                    &mut reader, 
                    bincode::config::standard()
                ).context("Failed to decode pantry database")?;
                
                db.ingredients = data;
            }
        }
        Ok(db)
    }

    pub fn save(&self) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        let file = File::create(&self.path).context("Failed to create pantry database file")?;
        let mut writer = BufWriter::new(file);
        
        bincode::serde::encode_into_std_write(
            &self.ingredients,
            &mut writer,
            bincode::config::standard()
        ).context("Failed to encode pantry database")?;
        
        Ok(())
    }

    pub fn store(&mut self, ingredient: Ingredient) -> Result<()> {
        // Validation could happen here
        self.ingredients.insert(ingredient.meta.name.clone(), ingredient);
        Ok(())
    }
    
    pub fn discard(&mut self, name: &str) -> Option<Ingredient> {
        self.ingredients.remove(name)
    }

    pub fn list(&self) -> Vec<&Ingredient> {
        let mut list: Vec<&Ingredient> = self.ingredients.values().collect();
        list.sort_by_key(|f| &f.meta.name);
        list
    }
    
    pub fn iter(&self) -> std::collections::hash_map::Values<'_, String, Ingredient> {
        self.ingredients.values()
    }
}
