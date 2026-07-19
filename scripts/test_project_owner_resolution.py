import importlib.util
import subprocess
import unittest
from pathlib import Path
from unittest.mock import patch

ROOT = Path(__file__).resolve().parent


def load_module(name: str, relative_path: str):
    spec = importlib.util.spec_from_file_location(name, ROOT / relative_path)
    module = importlib.util.module_from_spec(spec)
    assert spec and spec.loader
    spec.loader.exec_module(module)
    return module


preflight = load_module("preflight_project_token", "preflight_project_token.py")
governance = load_module("sync_github_governance", "sync_github_governance.py")
structure = load_module("sync_github_structure", "sync_github_structure.py")


class OwnerResolutionTests(unittest.TestCase):
    def test_preflight_owner_candidates_try_manifest_then_me_then_none(self):
        self.assertEqual(
            preflight.owner_candidates("nomed"),
            [["--owner", "nomed"], ["--owner", "@me"], []],
        )

    def test_governance_falls_back_to_owner_me(self):
        first = subprocess.CalledProcessError(
            returncode=1,
            cmd=["gh", "project", "view"],
            stderr="unknown owner type",
        )
        with patch.object(governance, "gh_json", side_effect=[first, {"id": "PVT_test"}]):
            view, owner_args = governance.resolve_project_owner_args("4", "nomed", "token")

        self.assertEqual(view["id"], "PVT_test")
        self.assertEqual(owner_args, ["--owner", "@me"])

    def test_structure_falls_back_to_owner_me(self):
        with patch.object(
            structure,
            "gh",
            side_effect=[
                structure.SyncError("unknown owner type"),
                {"id": "PVT_test"},
            ],
        ):
            view, owner_args = structure.resolve_project_owner_args("token", "4", "nomed")

        self.assertEqual(view["id"], "PVT_test")
        self.assertEqual(owner_args, ["--owner", "@me"])


if __name__ == "__main__":
    unittest.main()
