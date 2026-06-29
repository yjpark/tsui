{
  description = "Tsui — a data-driven, multi-backend GUI environment";

  inputs.jig.url = "github:edger-dev/jig";

  outputs = { self, jig }:
    jig.lib.mkWorkspace
      {
        pname = "tsui";
        src = ./.;
      }
      {
        kinora = {};
      };
}
