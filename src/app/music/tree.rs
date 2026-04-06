use std::{
    cell::RefCell,
    collections::HashMap,
    fs,
    path::PathBuf,
    rc::{Rc, Weak},
};

use crate::SUPPORTED_FORMATS;

#[derive(Debug, PartialEq)]
pub enum NodeType {
    File,
    Folder,
}

#[derive(Debug)]
pub struct Node {
    // Location of file/folder
    pub name: String,
    pub extension: Option<String>,
    pub node_type: NodeType,
    pub children: Option<Vec<Rc<RefCell<Node>>>>,
    pub parent: Option<Weak<RefCell<Node>>>,
}

impl Node {
    pub fn new(
        name: String,
        extension: Option<String>,
        node_type: NodeType,
        parent: Option<Weak<RefCell<Node>>>,
    ) -> Self {
        Self {
            name,
            extension,
            node_type,
            children: None,
            parent: parent,
        }
    }
    fn set_children(
        &mut self,
        dir_name: PathBuf,
        this: &Rc<RefCell<Node>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let children = if self.node_type == NodeType::File {
            None
        } else {
            let mut children: Vec<Rc<RefCell<Node>>> = Vec::new();

            for entry in fs::read_dir(dir_name)? {
                let entry = entry?;
                let node_type = if entry.path().is_dir() {
                    NodeType::Folder
                } else {
                    NodeType::File
                };
                let extension =
                    if let Some(extension) = entry.path().extension().and_then(|e| e.to_str()) {
                        if SUPPORTED_FORMATS.contains(&extension) {
                            Some(extension.to_string())
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                let node_name = entry.file_name().to_str().unwrap_or("").to_string();

                if (node_type == NodeType::File && extension != None)
                    || (node_type == NodeType::Folder && extension == None)
                {
                    let child = Rc::new(RefCell::new(Node::new(
                        node_name.clone(),
                        extension,
                        node_type,
                        Some(Rc::downgrade(this)),
                    )));
                    children.push(child);
                }
            }
            if children.is_empty() {
                None
            } else {
                Some(children)
            }
        };

        self.children = children;
        Ok(())
    }

    pub fn explore(
        &mut self,
        parent_location: Option<&str>,
        this: &Rc<RefCell<Node>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.node_type == NodeType::Folder {
            let dir_name = if let Some(pl) = parent_location {
                let mut p = PathBuf::from(pl);
                p.push(&self.name);
                p
            } else {
                PathBuf::from(&self.name)
            };

            self.set_children(dir_name.clone(), this)?;
        }
        Ok(())
    }
}
