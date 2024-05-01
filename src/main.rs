use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;

struct Noeud { // Node -> Noeud
    id: i32,
    distance: i32,
    precedent: Option<i32>,
}

impl Noeud {
    fn nouveau(id: i32) -> Self { // new -> nouveau
        Self {
            id,
            distance: i32::MAX,
            precedent: None,
        }
    }
}

struct Graphe { // Graph -> Graphe
    noeuds: HashMap<i32, Noeud>,
    arcs: HashMap<(i32, i32), i32>, // edges -> arcs
}

impl Graphe {
    fn nouveau() -> Self { // new -> nouveau
        Self {
            noeuds: HashMap::new(),
            arcs: HashMap::new(),
        }
    }

    fn ajouter_noeud(&mut self, id: i32) {
        self.noeuds.insert(id, Noeud::nouveau(id));
    }

    fn ajouter_arc(&mut self, source: i32, destination: i32, poids: i32) {
        self.arcs.insert((source, destination), poids);
    }

    fn dijkstra(&self, depart: i32, arrivee: i32) -> Option<Vec<i32>> {
        let mut distances: HashMap<i32, i32> = HashMap::new();
        for noeud in self.noeuds.values() {
            distances.insert(noeud.id, i32::MAX);
        }
        distances.insert(depart, 0);

        let mut non_visites: HashSet<i32> = HashSet::new();
        for noeud in self.noeuds.values() {
            non_visites.insert(noeud.id);
        }

        let mut tas: BinaryHeap<(i32, i32)> = BinaryHeap::new();
        tas.push((0, depart));

        while let Some((distance_courante, noeud_courant)) = tas.pop() {

            if noeud_courant == arrivee {

                let mut chemin = Vec::new();
                let mut noeud = noeud_courant;

                while let Some(precedent) = self.noeuds[&noeud].precedent {

                    chemin.push(noeud);
                    noeud = precedent;
                }

                chemin.push(depart);
                chemin.reverse();
                return Some(chemin);
            }

            if non_visites.contains(&noeud_courant) {
                non_visites.remove(&noeud_courant);

                for (voisin, poids) in self.arcs.iter().filter(|(_, poids_arc)| *poids_arc <= distance_courante + poids) {

                    let nouvelle_distance = distance_courante + poids;

                    if nouvelle_distance < distances[voisin] {

                        distances.insert(*voisin, nouvelle_distance);

                        self.noeuds.get_mut(voisin).unwrap().distance = nouvelle_distance;

                        self.noeuds.get_mut(voisin).unwrap().precedent = Some(noeud_courant);

                        tas.push((nouvelle_distance, *voisin));
                    }
                }
            }
        }

        None
    }
}

fn main() {
    let mut graphe = Graphe::nouveau();

    graphe.ajouter_noeud(1);
    graphe.ajouter_noeud(2);
    graphe.ajouter_noeud(3);
    graphe.ajouter_noeud(4);

    graphe.ajouter_arc(1, 2, 10);
    graphe.ajouter_arc(1, 3, 5);
    graphe.ajouter_arc(2, 3, 1);
    graphe.ajouter_arc(2, 4, 8);
    graphe.ajouter_arc(3, 4, 2);

    let chemin = graphe.dijkstra(1, 4);
    if let Some(chemin) = chemin {
        println!("Chemin le plus court: {:?}", chemin);
    } else {
        println!("Pas de chemin trouvé");
    }
}