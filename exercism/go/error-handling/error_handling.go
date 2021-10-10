package erratum

func Use(ro ResourceOpener, s string) error {
	resource, err := ro()
	defer resource.Close()

	return nil
}
