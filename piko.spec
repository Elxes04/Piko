Name:           piko
Version:        0.2.1
Release:        1%{?dist}
Summary:        A minimal, customizable system information tool

License:        MIT
URL:            https://github.com/Elxes04/piko
Source0:        %{url}/archive/v%{version}/%{name}-%{version}.tar.gz

BuildArch:      noarch
BuildRequires:  rust
BuildRequires:  cargo

%description
Piko is a lightweight and extensible command-line tool that gathers
and displays system information in a customizable format.
Inspired by Neofetch, it offers a clean and flexible way to view
details about your system — from OS to CPU, memory, and more.

%prep
%autosetup

%build
cargo build --release

%install
mkdir -p %{buildroot}%{_bindir}
mkdir -p %{buildroot}%{_datadir}/%{name}
mkdir -p %{buildroot}%{_docdir}/%{name}
mkdir -p %{buildroot}%{_licensedir}/%{name}

install -m755 target/release/%{name} %{buildroot}%{_bindir}/%{name}
install -m644 config/*.toml %{buildroot}%{_datadir}/%{name}/
install -m644 config/*.md %{buildroot}%{_datadir}/%{name}/
install -m644 README.md %{buildroot}%{_docdir}/%{name}/
install -m644 LICENSE %{buildroot}%{_licensedir}/%{name}/LICENSE

%files
%license LICENSE
%doc README.md
%{_bindir}/%{name}
%{_datadir}/%{name}/*.toml
%{_datadir}/%{name}/*.md

%changelog
* Mon Jan 01 2024 Elxes04 <elxes04@example.com> - 0.2.1-1
- Initial package release
