{
  description = "Flake utils demo";

  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      with pkgs;
      rec {
        packages.default = packages.akshare;

        packages.akshare = python3Packages.buildPythonPackage rec {
          pname = "akshare";
          version = "1.16.20";

          src = fetchPypi {
            inherit pname version;
            hash = "sha256-j2TcyVHalqyQPmLOzrf8ZG4vGJcYy4RlKe1BJJ8Uszw=";
          };

          build-system = with python3Packages; [ setuptools ];

          dependencies = with python3Packages; [
            beautifulsoup4
            lxml
            pandas
            requests
            urllib3
            html5lib
            tqdm
            xlrd
            openpyxl
            jsonpath
            tabulate
            decorator
            packages.mini-racer-bin
          ];
        };

        packages.mini-racer-bin = python3Packages.buildPythonPackage rec {
          pname = "mini_racer";
          version = "0.12.4";
          format = "wheel";

          src = fetchPypi {
            inherit pname version format;
            python = "py3";
            platform = "manylinux_2_31_x86_64";
            hash = "sha256-aaHETQKpBpuIFoTO8VotdH/gdD3ynq3Igf2nACquX9I=";
          };
        };

        # NOTE: 以 mini-racer 構建較難，暫直接用 wheel 打包 (mini-racer-bin)
        # packages.mini-racer = python3Packages.buildPythonPackage rec {
        #   pname = "mini-racer";
        #   version = "0.12.4";
        #   src = fetchPypi {
        #     inherit version;
        #     pname = "mini_racer";
        #     hash = "sha256-hMZ1U86fNzbUxhfYo/iClJ03pGz7R/4R2rM91nBOYqQ=";
        #   };
        #   pyproject = true;
        #   build-system = with python3Packages; [
        #     hatch
        #     hatch-fancy-pypi-readme
        #     packages.hatch-mkdocs
        #   ];
        #   dependencies = with python3Packages; [
        #     httplib2
        #     packaging
        #   ];
        #   nativeBuildInputs = [
        #     git
        #   ];
        # };
        # packages.hatch-mkdocs = python3Packages.buildPythonPackage rec {
        #   pname = "hatch-mkdocs";
        #   version = "0.1.0";
        #   src = fetchPypi {
        #     inherit version;
        #     pname = "hatch_mkdocs";
        #     hash = "sha256-unda4RQ1x57+tTmDHS3eKfeN4H1g8Sa2dt3VsjARAGk=";
        #   };
        #   pyproject = true;
        #   build-system = with python3Packages; [ hatch ];
        #   dependencies = with python3Packages; [ mkdocs-get-deps ];
        # };

        devShells.default =
          let
            python3 = pkgs.python3.withPackages (ppkgs: [
              packages.akshare
              ppkgs.pyarrow
            ]);
          in
          mkShell {
            buildInputs = [
              python3
            ];

            # 固定 PyO3 環境變量
            PYO3_PYTHON = "${python3}/bin/python3";
          };
      }
    );
}
