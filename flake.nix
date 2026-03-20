{
  description = "Sentry: NIST 800-53 Continuous Monitoring Stack";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux"; 
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          docker          # Container Engine
          docker-compose  # Orchestration
          go-task         # The Actuator
          prometheus      # TSDB for testing
          grafana         # Visualization for testing
          git             # Version Control
        ];

        shellHook = ''
          echo "--- 👁️ SENTRY OBSERVABILITY ENGINE LOADED ---"
          echo "Identity: Richard J. Mussell | Architect of the Zero-Lag Civilization"
          echo "Target: NIST 800-53 (CA-7) Continuous Monitoring"
          docker --version
        '';
      };
    };
}